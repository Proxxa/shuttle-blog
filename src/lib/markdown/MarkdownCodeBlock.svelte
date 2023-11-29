<!-- Black magic script. -->
<script lang="ts">
    import { marked } from "marked";
    let code_hack, code_hack2;
    $: if (code_hack) {
        code_hack2 = document.createElement("pre");
        code_hack2.innerHTML = marked.parse(code_hack.innerText);
        code_hack2 = code_hack2.children[0];
        code_hack2 = code_hack2.outerHTML;
    }
</script>

<span bind:this={code_hack} style="display:none;"><slot /></span>
{@html code_hack2}
