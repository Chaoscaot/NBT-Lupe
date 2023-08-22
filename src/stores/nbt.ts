import {writable} from "svelte/store";

export type NBT = ["ByteArray" | "IntArray" | "LongArray", number[]] | ["List", NBT[]] | ["Compound", { [key: string]: NBT }] | ["String", string] | ["Int" | "Long" | "Short" | "Byte" | "Double" | "Float", number]

export const globalNbt = writable<{[key: string]: NBT} | null>(null)
