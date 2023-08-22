<script lang="ts">
    import NBTTag from "./NBTTag.svelte";
    import TypeIcon from "./TypeIcon.svelte";

    export let name: string;
    export let nbt: Record<string, [string, any]>
    export let path: string[];
</script>


<li>
    <details open={Object.entries(nbt).length < 15}>
        <summary><TypeIcon char="C"/>{name}</summary>
        <ul>
            {#each Object.entries(nbt) as [key, tag]}
                <NBTTag type={tag[0]} name={key} nbt={tag[1]} path={[...path, key]} />
            {/each}
        </ul>
    </details>
</li>
