<!-- Black magic script. -->
<script>
    // @ts-ignore
    import { marked } from "marked";
    import { Link } from "svelte-navigator";
    import MarkdownCodeSpan from "./MarkdownCodeSpan.svelte";
    import { Render, createRender } from "svelte-render";
    import SvelteMarkdown from "svelte-markdown";
    import MarkdownCodeBlock from "./MarkdownCodeBlock.svelte";
    export let href, title, text;
    let hack;
    const textRender = createRender(SvelteMarkdown, { source: text, renderers: { code: MarkdownCodeBlock, codespan: MarkdownCodeSpan }});
    
    // @ts-ignore
    let additionalClasses = [];
    let finalText, finalClasses;
   // @ts-ignore
    $: if (hack) {
        let element = document.createElement("p");
        element.innerHTML = hack.innerHTML;

        while (element.children.length == 1 && element.children[0].tagName == element.tagName)
            // @ts-ignore
            element = element.children[0].cloneNode(true);

        function recurChildren(callback, element) {
            if (callback(element))
                return;
            for (const child in element.children)
                recurChildren(callback, element.children[child]);
        }

        let tagname = document.createElement("code").tagName;
        recurChildren(el => {
            if (el.tagName == tagname) {
                element.className += " codelink";
                return true;
            }
        }, element);

        finalText = element.innerText;
        finalClasses = element.className;
    }
</script>

<span style="display:none;" bind:this={hack}><Render of={textRender} /></span>
<div class="scroll-x">
    <img src={href} {title} alt={finalText} class={finalClasses}/>
</div>