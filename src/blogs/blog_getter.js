const blog_list = [
    "000-hello-world"
].map(k => "/blogs/".concat(k, ".md"));

const BAD_ID_MARKDOWN = `
# No such blog post.

You've discovered the special blog 404 page!
This is custom due to how blogs are loaded. 
If you're interested in how it's loaded, feel
free to check out the [Github](https://github.com/proxxa/shuttle-blog).
`;

/**
 * 
 * @param {any} id 
 * @returns {Promise<String>}
 */
export function import_blog(id) {
    if (!(id in blog_list))
        return new Promise(s => s(BAD_ID_MARKDOWN));
    return new Promise((s,j) => {
        fetch(blog_list[id])
            .then(r => r.text())
            .then(s)
            .catch(j)
    })
}