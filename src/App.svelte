<script lang="ts">
    import {dialog} from "@tauri-apps/api";
    import {invoke} from "@tauri-apps/api/tauri";
    import NBTCompoundTag from "./lib/NBTCompoundTag.svelte";
    import {globalNbt} from "./stores/nbt";
    import {onMount} from "svelte";
    import {appWindow} from "@tauri-apps/api/window";

    onMount(() => {
        appWindow.onMenuClicked(event => {
            switch (event.payload) {
                case "open": {
                    handleClick()
                    break
                }
            }
        })
    })

    async function handleClick() {
        let file = await dialog.open({
            directory: false,
            multiple: false,
            recursive: false,
            title: "Open a NBT file"
        })
        if (file) {
            let res = await invoke("load", {file: file})
            globalNbt.set(res as any)
        }
    }
</script>

<div class="card">
    Test!
</div>

{#if $globalNbt}
    <ul class="menu overflow-hidden">
        <NBTCompoundTag name="root" nbt={$globalNbt} path={[]}/>
    </ul>
{/if}
