class FooterMano extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        const shadow = this.attachShadow({ mode: "open" })
        let html;
        let styles; 

        styles = /* css */ `
            :host {
                background-color: #495159;
                height: 8rem;
                width: 100%;
                display: flex;
                flex-direction: column;
                gap: 1rem;
                align-items: center;
                justify-content: center;
                color: #eee
            }

            p {
                margin: 0;
                padding: 0;

            }
        `

        html = /* html */ `
            <style>
                ${styles}
            </style>

            <p>Created by Antonio Arias</p>
            <p>@2025 <b>MANO</b></p>
            <div class="links">
                <a href="#"><img src="" alt="github"></a>
                <a href="#"><img src="" alt="twitter"></a>
            </div>
        `

        shadow.innerHTML = html
    }

    
}

customElements.define('footer-mano', FooterMano)


