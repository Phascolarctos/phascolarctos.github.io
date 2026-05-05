import './style.css'
import init, { render } from '../pkg/koala_wasm_markdown.js'
import { fetchPostIndex} from './blog'
import 'github-markdown-css/github-markdown.css'
const loadPage = async () => {

  const posts = await fetchPostIndex();
  const app = document.getElementById('app')!;

  const urlParams = new URLSearchParams(window.location.search);
  const postId = urlParams.get('id');

  if (postId) {
    const post = posts.find(p => p.id === postId);
    if (post) {
      app.innerHTML = post.content;
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
