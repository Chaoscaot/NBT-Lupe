<script lang="ts">
    import NBTTag from "./NBTTag.svelte";
    import TypeIcon from "./TypeIcon.svelte";
    import {changed, getDefault, globalNbt} from "../stores/nbt";
    import type {NBT} from "../stores/nbt";
    import ContextMenu from "./ContextMenu.svelte";

    export let name: string;
    let nameEdit = name;
    export let nbt: Record<string, [string, any]>
    export let path: string[];
    export let arrayChild: boolean = false;

    let dialog: HTMLDialogElement;

    function addTag(type: string) {
        return () => {
            globalNbt.update(v => {
                let current: NBT = path.length > 0 ? v[path[0]] : null;
                let parent: { [key: string]: NBT } = v;
                for (let i = 1; i < path.length; i++) {
                    if (current["0"] === "List") {
                        current = current["1"][path[i]];
                    } else if (current["0"] === "Compound") {
                        parent = current["1"];
                        current = current["1"][path[i]];
                    } else {
                        current = current["1"][path[i]];
                    }
                }

                if (current) {
                    if (current["0"] === "Compound") {
                        parent = current["1"];
                    }
                }

                for (let i = 0; i < 99; i++) {
                    if (!Object.keys(parent).some(value => value === "New" + type + (i == 0?'':i))) {
                        parent["New" + type + (i == 0?'':i)] = [type as any, getDefault(type)];
                        break;
                    }
                }
                return v;
            })
            context.close();
            changed.set(true);
        }
    }

    function del() {
        globalNbt.update(v => {
            let current: NBT = v[path[0]];
            let parent: { [key: string]: NBT } | NBT[] = v;
            for (let i = 1; i < path.length; i++) {
                if (current["0"] === "List") {
                    parent = current["1"];
                    current = current["1"][path[i]];
                } else if (current["0"] === "Compound") {
                    parent = current["1"];
                    current = current["1"][path[i]];
                } else {
                    current = current["1"][path[i]];
                }
            }

            if (arrayChild && Array.isArray(parent)) {
                parent.splice(parseInt(name), 1);
            } else {
                delete parent[name];
            }

            return v;
        })
        context.close();
        changed.set(true);
    }

    function openRename() {
        context.close();
        dialog.showModal();
    }

    function rename() {
        dialog.close();
        globalNbt.update(v => {
            let current: NBT = v[path[0]];
            let parent: { [key: string]: NBT } = v;
            for (let i = 1; i < path.length; i++) {
                if (current["0"] === "List") {
                    current = current["1"][path[i]];
                } else if (current["0"] === "Compound") {
                    parent = current["1"];
                    current = current["1"][path[i]];
                } else {
                    current = current["1"][path[i]];
                }
            }

            let val = parent[name];
            delete parent[name];
            parent[nameEdit] = val;

            return v;
        })
        changed.set(true);
    }

    function reset() {
        nameEdit = name;
        dialog.close();
    }

    let context: ContextMenu;
</script>

<li>
    <details open={Object.entries(nbt).length < 15}>
        <summary on:contextmenu|preventDefault={context.openContext}><TypeIcon char="C"/>{name} [{Object.keys(nbt).length}]</summary>
        <ul>
            {#each Object.entries(nbt) as [key, tag]}
                <NBTTag type={tag[0]} name={key} nbt={tag[1]} path={[...path, key]} />
            {/each}
        </ul>
    </details>
</li>

<dialog bind:this={dialog} class="modal">
    <form class="modal-box" on:submit|preventDefault>
        <h1 class="mb-4">Edit {name}</h1>
        <div class="flex flex-col gap-4">
            <label class="label">Name</label>
            <input type="text" bind:value={nameEdit} class="input input-bordered w-full max-w-xs" />
        </div>
        <div class="modal-action">
            <button class="btn" on:click={reset}>Cancel</button>
            <button class="btn btn-primary" on:click={rename}>Save</button>
        </div>
    </form>
    <form method="dialog" class="modal-backdrop" on:submit|preventDefault={() => dialog.close()}>
        <button>close</button>
    </form>
</dialog>

<ContextMenu bind:this={context}>
    <li class="menu-title">{path.length > 0 ? path.join("/") : name}</li>
    <li><a href="#" on:click|preventDefault={addTag("Compound")}><TypeIcon char="C" />Add Compound</a></li>
    <li><a href="#" on:click|preventDefault={addTag("List")}><TypeIcon char="L" />Add List</a></li>
    <li><a href="#" on:click|preventDefault={addTag("Byte")}><TypeIcon char="B" />Byte</a></li>
    <li><a href="#" on:click|preventDefault={addTag("Short")}><TypeIcon char="S" />Short</a></li>
    <li><a href="#" on:click|preventDefault={addTag("Int")}><TypeIcon char="I" />Int</a></li>
    <li><a href="#" on:click|preventDefault={addTag("Long")}><TypeIcon char="L" />Long</a></li>
    <li><a href="#" on:click|preventDefault={addTag("Float")}><TypeIcon char="F" />Float</a></li>
    <li><a href="#" on:click|preventDefault={addTag("Double")}><TypeIcon char="D" />Double</a></li>
    <li><a href="#" on:click|preventDefault={addTag("String")}><TypeIcon char="S" />String</a></li>
    <li><a href="#" on:click|preventDefault={addTag("ByteArray")}><TypeIcon char="BA" />ByteArray</a></li>
    <li><a href="#" on:click|preventDefault={addTag("IntArray")}><TypeIcon char="IA" />IntArray</a></li>
    <li><a href="#" on:click|preventDefault={addTag("LongArray")}><TypeIcon char="LA" />LongArray</a></li>
    {#if path.length > 0}
        {#if !arrayChild}
            <li><a href="#" on:click|preventDefault={openRename}><TypeIcon color="bg-blue-500" char="R" />Rename</a></li>
        {/if}
        <li><a href="#" on:click|preventDefault={del}><TypeIcon color="bg-red-500" char="D" />Delete</a></li>
    {/if}
</ContextMenu>
