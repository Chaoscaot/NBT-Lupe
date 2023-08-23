<script lang="ts">
    import NBTTag from "./NBTTag.svelte";
    import TypeIcon from "./TypeIcon.svelte";
    import ContextMenu from "./ContextMenu.svelte";
    import {getDefault, globalNbt} from "../stores/nbt";
    import type {NBT} from "../stores/nbt";

    export let name: string;
    export let nbt: [string, any][];
    export let path: string[];

    let context: ContextMenu;

    function getThis(v: { [key: string]: NBT }): NBT {
        let current: NBT = v[path[0]];
        for (let i = 1; i < path.length; i++) {
            current = current["1"][path[i]];
        }

        return current;
    }

    function getParent(v: { [key: string]: NBT }): { [key: string]: NBT } | NBT[] {
        let current: NBT = v[path[0]];
        let parent: { [key: string]: NBT } | NBT[] = v;
        for (let i = 1; i < path.length; i++) {
            if (current["0"] === "List" || current["0"] === "Compound") {
                parent = current["1"];
            }
            current = current["1"][path[i]];
        }

        return parent;
    }

    function addTagStart(type: string) {
        return () => {
            globalNbt.update(v => {
                let current: NBT = getThis(v);

                if (current["0"] === "List") {
                    current["1"].unshift([type as any, getDefault(type)]);
                }

                return v;
            })
            context.close();
        }
    }

    function addTagEnd(type: string) {
        return () => {
            globalNbt.update(v => {
                let current: NBT = getThis(v);

                if (current["0"] === "List") {
                    current["1"].push([type as any, getDefault(type)]);
                }

                return v;
            })
            context.close();
        }
    }

    function del() {
        globalNbt.update(v => {
            let parent = getParent(v);

            if (Array.isArray(parent)) {
                parent.splice(parseInt(path[path.length - 1]), 1);
            } else {
                delete parent[path[path.length - 1]];
            }

            return v;
        })
        context.close();
    }
</script>

<li id={path.join("/")}>
    <details>
        <summary on:contextmenu|preventDefault={context.openContext}><TypeIcon char="L" />{name} [{nbt.length}]</summary>
        <ul>
            {#each nbt as tag, i}
                <NBTTag type={tag[0]} nbt={tag[1]} name={i.toString()} path={[...path, i.toString()]} listChildren={true} />
            {/each}
        </ul>
    </details>
</li>

<ContextMenu bind:this={context}>
    <li class="menu-title">{name}</li>
    <li>
        <details>
            <summary><TypeIcon char="S" />Add at start</summary>
            <ul>
                <li><a href="#" on:click|preventDefault={addTagStart("Compound")}><TypeIcon char="C" />Compound</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("List")}><TypeIcon char="L" />List</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("Byte")}><TypeIcon char="B" />Byte</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("Short")}><TypeIcon char="S" />Short</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("Int")}><TypeIcon char="I" />Int</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("Long")}><TypeIcon char="L" />Long</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("Float")}><TypeIcon char="F" />Float</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("Double")}><TypeIcon char="D" />Double</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("String")}><TypeIcon char="S" />String</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("ByteArray")}><TypeIcon char="BA" />ByteArray</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("IntArray")}><TypeIcon char="IA" />IntArray</a></li>
                <li><a href="#" on:click|preventDefault={addTagStart("LongArray")}><TypeIcon char="LA" />LongArray</a></li>
            </ul>
        </details>
    </li>
    <li>
        <details>
            <summary><TypeIcon char="E" />Add at end</summary>
            <ul>
                <li><a href="#" on:click|preventDefault={addTagEnd("Compound")}><TypeIcon char="C" />Compound</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("List")}><TypeIcon char="L" />List</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("Byte")}><TypeIcon char="B" />Byte</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("Short")}><TypeIcon char="S" />Short</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("Int")}><TypeIcon char="I" />Int</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("Long")}><TypeIcon char="L" />Long</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("Float")}><TypeIcon char="F" />Float</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("Double")}><TypeIcon char="D" />Double</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("String")}><TypeIcon char="S" />String</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("ByteArray")}><TypeIcon char="BA" />ByteArray</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("IntArray")}><TypeIcon char="IA" />IntArray</a></li>
                <li><a href="#" on:click|preventDefault={addTagEnd("LongArray")}><TypeIcon char="LA" />LongArray</a></li>
            </ul>
        </details>
    </li>
    <li><a href="#" on:click|preventDefault={del}><TypeIcon color="bg-red-500" char="D" />Delete</a></li>
</ContextMenu>
