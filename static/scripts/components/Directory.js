class DirectoryMano extends HTMLElement {
    constructor() {
        super();
        this._titles = null;
    }

    static observedAttributes  = ["titles"]

    attributeChangedCallback(name, oldValue, newValue) {
        this._titles = JSON.parse(newValue);
        console.log(this._titles)
        this._updateRendering()
    }

    _updateRendering() {
        const shadow = this.attachShadow({ mode: "open" })
        let html;
        let styles; 

        styles = /* css */ `
            :host {
                position: sticky;
                top: 5rem;
                background-color: #495159;
                height: calc(100vh - 4rem - 1rem);
                width: 20%;
                display: flex;
                flex-direction: column;
                gap: 1rem;
                align-items: center;
                justify-content: center;
                color: #eee
            }

            nav {
                width: 100%;
                height: 100%;
                display: flex;
                flex-direction: column;

                a {
                    width: calc(100% - 2rem);
                    padding: 1rem;
                    color: #eee;
                    text-decoration: none;

                    &:hover {
                        background-color: #5e6770;
                    }
                }
            }
        `
        html = /* html */ `
            <style>
                ${styles}
            </style>
            <nav>

            </nav>
        `
        shadow.innerHTML = html

        const nav = shadow.querySelector("nav");
        for (let i = 0; i < this._titles[0].length; i++) {
            const title = this._titles[0][i];
            const path = this._titles[1][i];
            const a = document.createElement("a")
            a.href = `/docs/${path.replace('.md', '')}`
            a.innerText = title

            nav.appendChild(a)
        }
    }

    connectedCallback() {
        this._updateRendering()
    }   
}

customElements.define('directory-mano', DirectoryMano)


