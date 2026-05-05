// src/blog.ts

export interface Post {
    id: string;
    title: string;
    date: string;
    tags: string[];
    filename: string;
    content: string;
}

export async function fetchPostIndex(): Promise<Post[]> {
    const response = await fetch('/posts.json'); // Vite 开发服务器会自动指向 public/posts.json
    if (!response.ok) throw new Error('无法加载文章索引');
    return await response.json();
}