<script lang="ts">
    import TypeIcon from "./TypeIcon.svelte";
    import NBTTag from "./NBTTag.svelte";
    import {getThis, globalNbt} from "../stores/nbt";
    import HexViewer from "./HexViewer.svelte";

    export let name: string;
    export let data: number[];
    export let type: string;
    export let path: string[];
    export let listChildren: boolean = false;

    function moveUp(index: number) {
        return () => {
            globalNbt.update(v => {
                let parent = getThis(v, path);

                console.log(Array.isArray(parent))
                if (Array.isArray(parent)) {
                    let temp = parent["1"][index];
                    parent["1"][index] = parent["1"][index + 1];
                    parent["1"][index + 1] = temp;
                }
                console.log(v);

                return v;
            })
        }
    }

    function moveDown(index: number) {
        return () => {
            globalNbt.update(v => {
                let parent = getThis(v, path);

                if (Array.isArray(parent)) {
                    let temp = parent["1"][index];
                    parent["1"][index] = parent["1"][index - 1];
                    parent["1"][index - 1] = temp;
                }

                console.log(v);
                return v;
            })
        }
    }

    let hex: HexViewer;
</script>

<li id={path.join("/")}>
    {#if data.length < 50}
        <details open={data.length < 5}>
            <summary on:contextmenu|preventDefault><TypeIcon char={`${type.charAt(0)}A`} />{name} [{data.length}]</summary>
            <ul>
                {#each data as d, i}
                    <NBTTag name={"[" + i + "]"} type={type.replace("Array", "")} nbt={d} path={[...path, i.toString()]} arrayChildren={true}>
                        <svelte:fragment slot="context">
                            {#if i !== 0}
                                <li><a href="/" on:click|preventDefault={moveDown(i)}>
                                    <TypeIcon char="U" color="bg-amber-500"/>Move Up</a></li>
                            {/if}
                            {#if i !== data.length - 1}
                                <li><a href="/" on:click|preventDefault={moveUp(i)}>
                                    <TypeIcon char="D" color="bg-amber-500"/>Move Down</a></li>
                            {/if}
                        </svelte:fragment>
                    </NBTTag>
                {/each}
            </ul>
        </details>
    {:else}
        <a href="/" on:click|preventDefault={hex.open}><TypeIcon char={`${type.charAt(0)}A`} />{name} [{data.length}]</a>
    {/if}
</li>

<HexViewer bind:this={hex} bind:array={data} bound="Byte"/>
