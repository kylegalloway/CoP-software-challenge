async function loadDate(mmdd) {
    const res = await fetch(`/api/date/${mmdd}`);
    const data = await res.json();

    const entriesEl = document.getElementById('entries');
    const emptyEl = document.getElementById('empty-state');

    entriesEl.innerHTML = '';
    if (data.entries.length === 0) { emptyEl.hidden = false; return; }
    emptyEl.hidden = true;

    for (const entry of data.entries) {
        // Setup entry list item
        const li = document.createElement('li');
        li.className = 'entry';

        // Add title section
        const title = document.createElement('p');
        title.className = 'entry-title';
        title.textContent = entry.title;
        li.appendChild(title);
        // Add quote section if it exists
        if (entry.quote) {
            const quote = document.createElement('p');
            quote.className = 'entry-quote';
            quote.textContent = entry.quote;
            li.appendChild(quote);
        }
        // Add year badge if it exists
        if (entry.specific_year) {
            const badge = document.createElement('span');
            badge.className = 'year-badge';
            badge.textContent = entry.specific_year;
            li.appendChild(badge);
        }
        // Add image if it exists
        if (entry.image_url) {
            const image = document.createElement('img');
            image.className = 'entry-image';
            image.src = entry.image_url;
            li.appendChild(image);
        }
        entriesEl.appendChild(li);
    }
}

function getActiveSkin() {
    const params = new URLSearchParams(location.search);
    return params.get('skin') || localStorage.getItem('skin') || 'modern';
}

function setActiveSkin(slug) {
    localStorage.setItem('skin', slug);
    const params = new URLSearchParams(location.search);
    params.set('skin', slug);
    const newUrl = `${location.pathname}?${params.toString()}`;
    history.replaceState(null, '', newUrl);
}

let currentSkinScript = null;
function applySkin(skin) {
    document.getElementById('skin-css').href = `/skins/${skin.slug}/skin.css`;

    if (currentSkinScript !== null) {
        currentSkinScript.remove();
        currentSkinScript = null;
    }
    if (skin.hasScript) {
        const script = document.createElement('script');
        script.src = `/skins/${skin.slug}/skin.js`;
        document.body.appendChild(script);
        currentSkinScript = script;
    }
}

let skinsBySlug = {};
async function loadSkinRegistry() {
    const res = await fetch('/skins/registry.json');
    const skins = await res.json();

    const select = document.getElementById('skin-picker');
    for (const skin of skins) {
        skinsBySlug[skin.slug] = skin;
        const option = document.createElement('option');
        option.value = skin.slug;
        option.textContent = skin.label;
        select.appendChild(option);
    }
}

document.addEventListener('DOMContentLoaded', async () => {
    await loadSkinRegistry();

    const activeSlug = getActiveSkin();
    document.getElementById('skin-picker').value = activeSlug;
    applySkin(skinsBySlug[activeSlug]);

    const now = new Date();
    const month = String(now.getMonth() + 1).padStart(2, '0'); // 0-indexed
    const day = String(now.getDate()).padStart(2, '0');
    const year = String(now.getFullYear());

    let dateInput = document.getElementById('date-input');
    dateInput.value = `${year}-${month}-${day}`;
    loadDate(`${month}-${day}`);

    document.getElementById('skin-picker').addEventListener('change', (e) => {
        setActiveSkin(e.target.value);
        applySkin(skinsBySlug[e.target.value]);
    });

    dateInput.addEventListener('change', (e) => {
        const [year, month, day] = e.target.value.split('-');
        loadDate(`${month}-${day}`);
    });
});
