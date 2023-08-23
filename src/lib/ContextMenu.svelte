<script lang="ts">

    let context: HTMLUListElement;
    let backdrop: HTMLButtonElement;
    let x: string = "0";
    let xNumber: number = 0;
    let y: string = "0";
    let yNumber: number = 0;

    export function openContext(e: MouseEvent) {
        open();

        if (e.clientX + context.getBoundingClientRect().width > window.innerWidth) {
            xNumber = window.innerWidth - context.getBoundingClientRect().width;
        } else {
            xNumber = e.clientX;
        }

        if (e.clientY + context.getBoundingClientRect().height > window.innerHeight) {
            yNumber = window.innerHeight - context.getBoundingClientRect().height;
        } else {
            yNumber = e.clientY;
        }

        x = xNumber + "px";
        y = yNumber + "px";
    }

    function resize() {
        setTimeout(() => {
            if (xNumber + context.getBoundingClientRect().width > window.innerWidth) {
                xNumber = (window.innerWidth - context.getBoundingClientRect().width);
                x = xNumber + "px";
            }

            if (yNumber + context.getBoundingClientRect().height > window.innerHeight) {
                yNumber = (window.innerHeight - context.getBoundingClientRect().height);
                y = yNumber + "px";
            }
        }, 0)
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
<ul class="menu bg-base-200 w-56 rounded-box fixed z-20 overflow-scroll hidden" on:click={resize} style:top={y} style:left={x} bind:this={context}>
    <slot></slot>
</ul>
