type BlogMetadata = {
    id: string;
    title: string;
    author: string;
    description: string;
    image: string;
    ordering: number;
};

const emptyMetadata: BlogMetadata = {
    id: "",
    title: "",
    author: "",
    description: "",
    image: "",
    ordering: 0,
}

const fetchOptions: RequestInit = {
    redirect: "follow",
};

export async function import_blog(
    id: string
): Promise<{ text: string; meta: BlogMetadata }> {
    try {
        let text = await fetch(`/api/blog/${id}`).then((r) => r.text());
        let meta = await fetch(`/api/blogdata/${id}`).then((r) => r.json());
        return { text, meta };
    } catch (err) {
        return { text: "", meta: emptyMetadata }
    }
}

export async function list_blogs(): Promise<Map<String, BlogMetadata>> {
    return fetch(`/api/blogs`).then((r) => r.json());
}
