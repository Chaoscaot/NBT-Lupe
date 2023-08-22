import {get, writable} from "svelte/store";
import {appWindow} from "@tauri-apps/api/window";

export type NBT = ["ByteArray" | "IntArray" | "LongArray", number[]] | ["List", NBT[]] | ["Compound", { [key: string]: NBT }] | ["String", string] | ["Int" | "Long" | "Short" | "Byte" | "Double" | "Float", number]

export const globalNbt = writable<{[key: string]: NBT} | null>(null)

export const openFile = writable<string | null>(null)

export const changed = writable(false)
