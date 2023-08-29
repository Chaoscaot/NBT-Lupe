import {writable} from "svelte/store";

export type NBT = ["ByteArray" | "IntArray" | "LongArray", number[]] | ["List", NBT[]] | ["Compound", { [key: string]: NBT }] | ["String", string] | ["Int" | "Long" | "Short" | "Byte" | "Double" | "Float", number]

export const globalNbt = writable<{[key: string]: NBT} | null>({})

export const openFile = writable<string | null>(null)

export const changed = writable(false)

export function getDefault(type: string): any {
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

export function getThis(v: { [key: string]: NBT }, path: string[]): NBT {
    let current: NBT = v[path[0]];
    for (let i = 1; i < path.length; i++) {
        current = current["1"][path[i]];
    }

    return current;
}

export function getParent(v: { [key: string]: NBT }, path: string[]): { [key: string]: NBT } | NBT[] {
    let current: NBT = v[path[0]];
    let parent: { [key: string]: NBT } | NBT[] = v;
    for (let i = 1; i < path.length; i++) {
        if (current["0"] === "List" || current["0"] === "Compound") {
            parent = current["1"];
        }
        current = current["1"][path[i]];
    }

    return parent;
}
