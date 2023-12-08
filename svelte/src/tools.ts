export function safeId(id: string): HTMLElement {
    let el: HTMLElement | null = document.getElementById(id);
    if (el instanceof HTMLElement) {
        return el;
    }
    throw new Error("safeId attempted to get nonexistent element");
}