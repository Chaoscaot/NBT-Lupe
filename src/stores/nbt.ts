import {writable} from "svelte/store";

export type NBT = ["ByteArray" | "IntArray" | "LongArray", number[]] | ["List", NBT[]] | ["Compound", { [key: string]: NBT }] | ["String", string] | ["Int" | "Long" | "Short" | "Byte" | "Double" | "Float", number]

export const globalNbt = writable<{[key: string]: NBT} | null>({})

export const openFile = writable<string | null>(null)

export const changed = writable(false)

export function getDefault(type: string) {
    switch (type) {
        case "Compound":
            return {};
        case "ByteArray":
        case "IntArray":
        case "LongArray":
        case "List":
            return [];
        case "Byte":
        case "Short":
        case "Int":
        case "Long":
        case "Float":
        case "Double":
            return 0;
        case "String":
            return "";
    }
}
