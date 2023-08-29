<script lang="ts">
    import NBTCompoundTag from "./NBTCompoundTag.svelte";
    import NBTList from "./NBTList.svelte";
    import NBTValue from "./NBTValue.svelte";
    import NBTByteArray from "./NBTByteArray.svelte";

    export let type: string;
    export let name: string;
    export let nbt: any;
    export let path: string[];
    export let listChildren: boolean = false;
    export let arrayChildren: boolean = false;
</script>


{#if type === "Compound"}
    <NBTCompoundTag nbt={nbt} name={name} path={path} arrayChild={listChildren}>
        <svelte:fragment slot="context">
            <slot name="context"></slot>
        </svelte:fragment>
    </NBTCompoundTag>
{:else if type === "List"}
    <NBTList nbt={nbt} name={name} path={path} listChildren={listChildren}>
        <svelte:fragment slot="context">
            <slot name="context"></slot>
        </svelte:fragment>
    </NBTList>
{:else if type === "ByteArray" || type === "IntArray" || type === "LongArray"}
    <NBTByteArray bind:data={nbt} name={name} type={type} path={path} listChildren={listChildren}>
        <svelte:fragment slot="context">
            <slot name="context"></slot>
        </svelte:fragment>
    </NBTByteArray>
{:else}
    <NBTValue type={type} name={name} value={nbt} path={path} listChildren={listChildren} arrayChildren={arrayChildren}>
        <svelte:fragment slot="context">
            <slot name="context"></slot>
        </svelte:fragment>
    </NBTValue>
{/if}
