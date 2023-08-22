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
</script>


{#if type === "Compound"}
    <NBTCompoundTag nbt={nbt} name={name} path={path}/>
{:else if type === "List"}
    <NBTList nbt={nbt} name={name} path={path}/>
{:else if type === "ByteArray" || type === "IntArray" || type === "LongArray"}
    <NBTByteArray data={nbt} name={name} type={type} path={path} />
{:else}
    <NBTValue type={type} name={name} value={nbt} path={path} listChildren={listChildren} />
{/if}
