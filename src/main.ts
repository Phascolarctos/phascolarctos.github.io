import './style.css'
import init, { render } from '../pkg/koala_wasm_markdown.js'
import { fetchPostIndex} from './blog'

const loadPage = async () => {

  const posts = await fetchPostIndex();
  const app = document.getElementById('app')!;

  const urlParams = new URLSearchParams(window.location.search);
  const postId = urlParams.get('id');

  if (postId) {
    const post = posts.find(p => p.id === postId);
    if (post) {
      const res = await fetch(`/articles/${post.filename}`);
      const md = await res.text();
      const pureMd = md.replace(/^---[\s\S]*?---/, '');
      await init();
      const html_str = render(pureMd);
      app.innerHTML = html_str;
    }
  } else {
    app.innerHTML = `
      <h1>ming's doc</h1>
      <ul>
        ${posts.map(p => `
          <li>
            ${p.date} - <a href="?id=${p.id}">${p.title}</a> - ${p.tags}
          </li>
        `).join('')}
      </ul>
    `;
  }
}
loadPage();
