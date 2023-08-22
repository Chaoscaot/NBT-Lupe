<script lang="ts">

    let context: HTMLUListElement;
    let backdrop: HTMLButtonElement;
    let x: string = "0";
    let y: string = "0";

    export function openContext(e: MouseEvent) {
        open();

        if (e.clientX + context.getBoundingClientRect().width > window.innerWidth) {
            x = (window.innerWidth - context.getBoundingClientRect().width) + "px";
        } else {
            x = e.clientX + "px";
        }

        if (e.clientY + context.getBoundingClientRect().height > window.innerHeight) {
            y = (window.innerHeight - context.getBoundingClientRect().height) + "px";
        } else {
            y = e.clientY + "px";
        }
    }

    function open() {
        context.classList.remove("hidden")
        backdrop.classList.remove("hidden")
    }

    export function close() {
        context.classList.add("hidden")
        backdrop.classList.add("hidden")
    }
</script>

<button class="w-screen h-screen top-0 left-0 fixed z-10 hidden" on:click={close} on:contextmenu={close} bind:this={backdrop}></button>
<ul class="menu bg-base-200 w-56 rounded-box fixed z-20 overflow-scroll hidden" style:top={y} style:left={x} bind:this={context}>
    <slot></slot>
</ul>
