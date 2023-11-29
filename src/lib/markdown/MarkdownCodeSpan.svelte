<!-- Black magic script. -->
<script lang="ts">
    import { marked } from "marked";
    let code_hack, code_hack2;
    $: if (code_hack) {
        let el: Element = document.createElement("p");
        el.innerHTML = marked.parse(code_hack.innerText) as string

        el = el.getElementsByTagName("p")[0];
        let codeEl = el.children[0].cloneNode(true) as Element
        el = document.createElement("span")
        el.className = "inlinecode"
        el.appendChild(codeEl)
        code_hack2 = el.outerHTML
    }
</script>

<span bind:this={code_hack} style="display:none;"><slot/></span>
{@html code_hack2}
