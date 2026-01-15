class HeaderMano extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        const shadow = this.attachShadow({ mode: "closed" })
        let html;
        let styles; 

        styles = /* css */ `
            :host {
                position: sticky;
                width: calc(100%-4rem);
                display: flex;
                padding: .5rem 2rem;
                justify-content: space-between;
                align-items: center;
                font-family: 'Jaro';
                background-color: #fff;
                height: 4rem;
                top: 0;
                z-index: 3;
            }

            img {
                width: auto;
                height: 80%;
            }

            nav {
                height: 100%;
                display: flex;
                align-items: center;
                width: 50%;
                justify-content: space-evenly

            }

            a {
                color: #000;
                text-decoration: none;
            }
        `

        html = /* html */ `
            <style>
                ${styles}
            </style>

            <img src="/public/favicon.ico" alt="">
            <nav>
                <a href="/docs">DOCS</a>
                <a href="/calc">CALCULATOR</a>
                <a href="http://github.com/arkeasz/mano" >REPOSITORY</a>
            </nav>
        `

        shadow.innerHTML = html
    }

    
}

customElements.define('header-mano', HeaderMano)


