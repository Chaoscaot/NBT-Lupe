<script lang="ts">
    import NBTTag from "./NBTTag.svelte";
    import TypeIcon from "./TypeIcon.svelte";

    export let name: string;
    export let nbt: [string, any][];
    export let path: string[];
</script>

<li>
    <details>
        <summary><TypeIcon char="L" />{name}</summary>
        <ul>
            {#each nbt as tag, i}
                <NBTTag type={tag[0]} nbt={tag[1]} name={i.toString()} path={[...path, i.toString()]} />
            {/each}
        </ul>
    </details>
</li>
