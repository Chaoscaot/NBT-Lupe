<script lang="ts">
    import TypeIcon from "./TypeIcon.svelte";
    import {changed, globalNbt} from "../stores/nbt";
    import type {NBT} from "../stores/nbt";
    import {onMount} from "svelte";
    import ContextMenu from "./ContextMenu.svelte";

    export let name: string;
    let nameEdit: string = name;
    export let value: any;
    let valueEdit: any = value;
    export let type: string;
    let typeEdit: string = type;
    export let path: string[];
    export let listChildren: boolean = false;

    let dialog: HTMLDialogElement;
    let context: ContextMenu;

    function open() {
        dialog.showModal();
    }

    function save() {
        // Update the value, type and name to the new values at the given path
        globalNbt.update(v => {
            let current: NBT = v[path[0]];
            let parent: { [key: string]: NBT } = v;
            let listParent: number[] | null = null
            for (let i = 1; i < path.length; i++) {
                if (current["0"] === "List") {
                    current = current["1"][path[i]];
                } else if (current["0"] === "Compound") {
                    parent = current["1"];
                    current = current["1"][path[i]];
                } else if (current["0"] === "ByteArray" || current["0"] === "IntArray" || current["0"] === "LongArray") {
                    listParent = current["1"];
                    current = current["1"][path[i]];
                } else {
                    current = current["1"][path[i]];
                }
            }

            if (listChildren) {
                listParent[name] = valueEdit;
            } else {
                if(name != nameEdit) {
                    delete parent[name];
                }
                let v = typeEdit === "String" ? valueEdit : Number.parseInt(valueEdit);
                parent[nameEdit] = [typeEdit as any, v];
            }

            return v;
        })

        dialog.close();
        changed.set(true);
    }

    function del() {
        globalNbt.update(v => {
            let current: NBT = v[path[0]];
            let parent: { [key: string]: NBT } = v;
            let listParent: number[] | null = null
            for (let i = 1; i < path.length; i++) {
                if (current["0"] === "List") {
                    current = current["1"][path[i]];
                } else if (current["0"] === "Compound") {
                    parent = current["1"];
                    current = current["1"][path[i]];
                } else if (current["0"] === "ByteArray" || current["0"] === "IntArray" || current["0"] === "LongArray") {
                    listParent = current["1"];
                    current = current["1"][path[i]];
                } else {
                    current = current["1"][path[i]];
                }
            }

            if (listChildren) {
                delete listParent[name];
            } else {
                delete parent[name];
            }

            return v;
        })
        context.close();
        changed.set(true);
    }

    function reset() {
        dialog.close();
        nameEdit = name;
        valueEdit = value;
        typeEdit = type;
    }
</script>

<li>
    <a on:click={open} on:contextmenu|preventDefault={context.openContext}><TypeIcon char={type.at(0)} />{name}: {type === "String" ? `"${value}"` : value}</a>
</li>

<ContextMenu bind:this={context}>
    <li><a href="#" on:click|preventDefault={del}><TypeIcon color="bg-red-500" char="D" />Delete</a></li>
</ContextMenu>

<dialog bind:this={dialog} class="modal">
    <form class="modal-box" on:submit|preventDefault>
        <h1 class="mb-4">Edit {name}</h1>
        <div class="flex flex-col gap-4">
            {#if !listChildren}
                <label class="label">Name</label>
                <input type="text" bind:value={nameEdit} class="input input-bordered w-full max-w-xs" />
                <label class="label">Type</label>
                <select bind:value={typeEdit} class="select w-full max-w-xs select-bordered">
                    <option value="Byte">Byte</option>
                    <option value="Short">Short</option>
                    <option value="Int">Int</option>
                    <option value="Long">Long</option>
                    <option value="Float">Float</option>
                    <option value="Double">Double</option>
                    <option value="String">String</option>
                </select>
            {/if}
            <label class="label">Value</label>
            <input type="text" bind:value={valueEdit} class="input input-bordered w-full max-w-xs" />
        </div>
        <div class="modal-action">
            <button class="btn" on:click={reset}>Cancel</button>
            <button class="btn btn-primary" on:click={save}>Save</button>
        </div>
    </form>
    <form method="dialog" class="modal-backdrop" on:submit|preventDefault={() => dialog.close()}>
        <button>close</button>
    </form>
</dialog>

