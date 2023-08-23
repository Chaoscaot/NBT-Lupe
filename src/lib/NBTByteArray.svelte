<script lang="ts">
    import TypeIcon from "./TypeIcon.svelte";
    import NBTTag from "./NBTTag.svelte";

    export let name: string;
    export let data: number[];
    export let type: string;
    export let path: string[];
</script>

<li id={path.join("/")}>
    {#if data.length < 50}
        <details open={data.length < 5}>
            <summary><TypeIcon char={`${type.charAt(0)}A`} />{name} [{data.length}]</summary>
            <ul>
                {#each data as d, i}
                    <NBTTag name={i.toString()} type={type.replace("Array", "")} nbt={d} path={[...path, i]} listChildren={true} />
                {/each}
            </ul>
        </details>
    {:else}
        <a><TypeIcon char={`${type.charAt(0)}A`} />{name} [{data.length}]</a>
    {/if}
</li>
