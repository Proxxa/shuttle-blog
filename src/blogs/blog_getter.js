const blog_list = [
    "000-hello-world"
].map(k => "/blogs/".concat(k, ".md"));

/**
 * 
 * @param {number} id 
 * @returns {Promise<String>}
 */
export function import_blog(id) {
    return new Promise((s,j) => {
        fetch(blog_list[id])
            .then(r => r.text())
            .then(s)
            .catch(j)
    })
}