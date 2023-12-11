import {type Writable, writable} from "svelte/store";

export const sw: Writable<number> = writable(0);
export function wasmGetHome(): number {
    sw.set(1);
    return 0;
}
export const wasmIni: Writable<bool> = writable(false);