import {type Writable, writable} from "svelte/store";

export const sw: Writable<number> = writable(0);