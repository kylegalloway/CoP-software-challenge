function todayAsMMDD() {
    const now = new Date();
    const month = String(now.getMonth() + 1).padStart(2, '0'); // 0-indexed
    const day = String(now.getDate()).padStart(2, '0');
    return `${month}-${day}`;
}

async function loadDate(mmdd) {
    const res = await fetch(`/api/date/${mmdd}`);
    const data = await res.json();

    const entriesEl = document.getElementById('entries');
    const emptyEl = document.getElementById('empty-state');

    entriesEl.innerHTML = '';
    if (data.entries.length === 0) { emptyEl.hidden = false; return; }
    emptyEl.hidden = true;

    for (const entry of data.entries) {
        const li = document.createElement('li');
        li.textContent = entry.title + (entry.quote ? `: ${entry.quote}` : '');
        entriesEl.appendChild(li);
    }
}

function getActiveSkin() {
    const params = new URLSearchParams(location.search);
    return params.get('skin') || localStorage.getItem('skin') || 'modern';
}

function setActiveSkin(slug) {
    localStorage.setItem('skin', slug);
    // also consider updating the URL via history.replaceState, your call
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

    const mmdd = todayAsMMDD();
    loadDate(mmdd);

    document.getElementById('skin-picker').addEventListener('change', (e) => {
        setActiveSkin(e.target.value);
        applySkin(skinsBySlug[e.target.value]);
    });

    document.getElementById('date-input').addEventListener('change', (e) => {
        const [year, month, day] = e.target.value.split('-');
        loadDate(`${month}-${day}`);
    });
});
