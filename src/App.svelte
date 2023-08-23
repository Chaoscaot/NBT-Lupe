<script lang="ts">
    import {dialog, shell} from "@tauri-apps/api";
    import {invoke} from "@tauri-apps/api/tauri";
    import NBTCompoundTag from "./lib/NBTCompoundTag.svelte";
    import {changed, globalNbt, openFile} from "./stores/nbt";
    import {onMount} from "svelte";
    import {appWindow} from "@tauri-apps/api/window";
    import {getMatches} from "@tauri-apps/api/cli";

    onMount(() => {
        let unsub = appWindow.onMenuClicked(event => {
            switch (event.payload) {
                case "open": {
                    handleClick()
                    break
                }
                case "reload": {
                    if ($openFile) {
                        oFile($openFile)
                    }
                    break
                }
                case "save": {
                    if ($openFile) {
                        saveAs($openFile)
                    } else {
                        dialog.save({
                            title: "Save NBT file"
                        }).then(value => {
                            saveAs(value)
                            openFile.set(value)
                        })
                    }
                    break
                }
                case "save-as": {
                    dialog.save({
                        title: "Save NBT file"
                    }).then(value => {
                        saveAs(value)
                    })
                    break
                }
                case "gh": {
                    shell.open("https://github.com/Chaoscaot/NBT-Lupe")
                }
            }
        })

        getMatches().then(value => {
            console.log(
                value.args)
            let file = value.args.file;
            if (file.occurrences > 0) {
                oFile(file.value as string)
            }
        })

        return () => {
            unsub.then(value => value())
        }
    })

    window.addEventListener("keypress", ev => {
        if (ev.ctrlKey) {
            switch (ev.key) {
                case "s": {
                    if ($openFile) {
                        saveAs($openFile)
                    } else {
                        dialog.save({
                            title: "Save NBT file"
                        }).then(value => {
                            saveAs(value)
                            openFile.set(value)
                        })
                    }
                    break
                }
                case "o": {
                    handleClick()
                    break
                }
                case "r": {
                    if ($openFile) {
                        oFile($openFile)
                    }
                    break
                }
            }
        }
    })

    async function saveAs(file: string) {
        console.log("Saving to", file)
        await invoke("save_as_fun", {file: file, nbt: $globalNbt})
        changed.set(false)
    }

    async function oFile(file: string) {
        let res = await invoke("load", {file: file})
        globalNbt.set(res as any)
        openFile.set(file)
        changed.set(false)
    }

    async function handleClick() {
        let file = await dialog.open({
            directory: false,
            multiple: false,
            recursive: false,
            title: "Open a NBT file"
        })
        if (file) {
            await oFile(file as string)
        }
    }

    $: if ($openFile) {
        appWindow.setTitle("NBT Lupe - " + ($changed ? '*' : '') + $openFile)
    } else {
        appWindow.setTitle("NBT Lupe")
    }
</script>

<ul class="menu overflow-hidden">

    {#if $globalNbt}
        <NBTCompoundTag name="root" nbt={$globalNbt} path={[]}/>
    {:else}
        <NBTCompoundTag name="root" nbt={{}} path={[]}/>
    {/if}
</ul>
