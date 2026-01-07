# 🕊️ Acompañante Espiritual AI / Spiritual Companion AI

![Rust](https://img.shields.io/badge/Backend-Rust-black?style=flat-square&logo=rust)
![Status](https://img.shields.io/badge/Status-Production%20Ready-success?style=flat-square)
![License](https://img.shields.io/badge/License-Proprietary-blue?style=flat-square)
![Author](https://img.shields.io/badge/Author-Angel%20A.%20Urbina-d4af37?style=flat-square)

> **Diseñado por Angel A. Urbina (2026)**.
> Una plataforma de asistencia pastoral basada en Inteligencia Artificial, Espiritualidad Católica y privacidad.

---

### 🌐 Select Language / Selecciona Idioma / Selecciona Idioma

| [🇪🇸 Español](#-español) | [🇺🇸 English](#-english) | [🏴󠁥󠁳󠁣󠁴󠁿 Català](#-català) |
| :---: | :---: | :---: |

---

<a name="-español"></a>
## 🇪🇸 ESPAÑOL

### I. Para Agentes Pastorales y Usuarios
**¿Qué es esta herramienta?**
El "Acompañante Espiritual" es un sistema diseñado para ofrecer una primera acogida y reflexión a personas que buscan consuelo, claridad o discernimiento. No sustituye al sacerdote ni al acompañamiento humano, sino que actúa como una herramienta de apoyo para estructurar pensamientos y ofrecer una perspectiva basada en la tradición cristiana.

**Características Principales:**
*   **🎙️ Entrada y Salida por Voz:** Pensado para la accesibilidad. Puede dictar su inquietud pulsando el micrófono y escuchar la reflexión generada.
*   **🛡️ Privacidad Absoluta:** El sistema no guarda bases de datos persistentes de sus conversaciones. Los archivos subidos se procesan en memoria y se eliminan inmediatamente.
*   **📄 Exportación Profesional:** Puede descargar la reflexión en formato PDF (diseño tipo carta) o HTML para imprimirla o guardarla.
*   **✝️ Enfoque Católico:** El sistema está instruido para evitar juicios moralizantes, ofreciendo en su lugar acogida, sentido espiritual y referencias bíblicas de esperanza.
*   **🌐 Bilingüe:** Funciona nativamente en Español y Catalán.

**¿Cómo se usa?**
1.  Seleccione su idioma en la pantalla de bienvenida.
2.  Escriba o dicte su experiencia o inquietud actual.
3.  (Opcional) Adjunte un archivo (PDF/Word) si desea que el sistema analice un texto largo.
4.  Reciba una reflexión estructurada en cuatro puntos: Acompañamiento, Sentido, Orientación y Palabra de Vida.

---

### II. Documentación Técnica (Ingeniería)
**Arquitectura del Sistema**
El backend está construido en **Rust** utilizando `Actix-Web` para garantizar el máximo rendimiento, seguridad de memoria y concurrencia. El frontend es HTML5/CSS3 puro con renderizado en servidor (SSR) vía `Tera`.

**Requisitos Previos**
*   **Rust & Cargo:** v1.84 o superior.
*   **Librerías del Sistema:** `pkg-config`, `libssl-dev`, `libpoppler-glib-dev` (para procesamiento de PDFs).
*   **API Key:** Una clave válida de OpenAI (`OPENAI_API_KEY`).

**Instalación y Despliegue**

1.  **Configuración de Entorno:**
    Cree un archivo `.env` en la raíz:
    ```env
    OPENAI_API_KEY=sk-su-clave-aqui...
    AI_MODEL=gpt-4o-mini
    PORT=8080
    ```

2.  **Ejecución Local:**
    ```bash
    cargo run --release
    ```
    Acceda a: `http://localhost:8080`

3.  **Docker (Producción):**
    El proyecto incluye un `Dockerfile` *multi-stage* optimizado (basado en Debian Slim).
    ```bash
    docker build -t acompanante-espiritual .
    docker run -p 8080:8080 --env-file .env acompanante-espiritual
    ```

**Estructura de Prompts**
El sistema utiliza inyección de prompts dinámicos según el idioma seleccionado (`get_system_prompt`), forzando una estructura HTML estricta en la respuesta del LLM para garantizar el formato de salida.

---

<a name="-english"></a>
## 🇺🇸 ENGLISH

### I. For Pastoral Agents and Users
**What is this tool?**
The "Spiritual Companion" is a system designed to offer initial welcome and reflection to individuals seeking comfort, clarity, or discernment. It does not replace a priest or human accompaniment but acts as a support tool to structure thoughts and offer a perspective based on Christian tradition.

**Key Features:**
*   **🎙️ Voice Input & Output:** Designed for accessibility. You can dictate your concerns by pressing the microphone and listen to the generated reflection.
*   **🛡️ Absolute Privacy:** The system does not maintain persistent databases of your conversations. Uploaded files are processed in memory and deleted immediately.
*   **📄 Professional Export:** You can download the reflection in PDF (letter format) or HTML for printing or saving.
*   **✝️ Catholic Focus:** The system is instructed to avoid moralizing judgments, offering instead welcome, spiritual meaning, and biblical references of hope.
*   **🌐 Bilingual:** Works natively in Spanish and Catalan.

**How to use it?**
1.  Select your language on the welcome screen.
2.  Type or dictate your current experience or concern.
3.  (Optional) Attach a file (PDF/Word) if you want the system to analyze a longer text.
4.  Receive a structured reflection in four points: Welcome, Meaning, Guidance, and Word of Life.

---

### II. Technical Documentation (Engineering)
**System Architecture**
The backend is built in **Rust** using `Actix-Web` to ensure maximum performance, memory safety, and concurrency. The frontend is pure HTML5/CSS3 with server-side rendering (SSR) via `Tera`.

**Prerequisites**
*   **Rust & Cargo:** v1.84 or higher.
*   **System Libraries:** `pkg-config`, `libssl-dev`, `libpoppler-glib-dev` (for PDF processing).
*   **API Key:** A valid OpenAI key (`OPENAI_API_KEY`).

**Installation and Deployment**

1.  **Environment Setup:**
    Create a `.env` file in the root:
    ```env
    OPENAI_API_KEY=sk-your-key-here...
    AI_MODEL=gpt-4o-mini
    PORT=8080
    ```

2.  **Local Execution:**
    ```bash
    cargo run --release
    ```
    Access at: `http://localhost:8080`

3.  **Docker (Production):**
    The project includes an optimized *multi-stage* `Dockerfile` (based on Debian Slim).
    ```bash
    docker build -t spiritual-companion .
    docker run -p 8080:8080 --env-file .env spiritual-companion
    ```

**Prompt Engineering**
The system uses dynamic prompt injection based on the selected language (`get_system_prompt`), enforcing a strict HTML structure in the LLM response to guarantee the output format.

---

<a name="-català"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 CATALÀ

### I. Per a Agents Pastorals i Usuaris
**Què és aquesta eina?**
L'"Acompanyant Espiritual" és un sistema dissenyat per oferir una primera acollida i reflexió a persones que cerquen consol, claredat o discerniment. No substitueix el sacerdot ni l'acompanyament humà, sinó que actua com una eina de suport per estructurar pensaments i oferir una perspectiva basada en la tradició cristiana.

**Característiques Principals:**
*   **🎙️ Entrada i Sortida per Veu:** Pensat per a l'accessibilitat. Podeu dictar la vostra inquietud prement el micròfon i escoltar la reflexió generada.
*   **🛡️ Privacitat Absoluta:** El sistema no guarda bases de dades persistents de les vostres converses. Els fitxers pujats es processen en memòria i s'eliminen immediatament.
*   **📄 Exportació Professional:** Podeu descarregar la reflexió en format PDF (disseny tipus carta) o HTML per imprimir-la o guardar-la.
*   **✝️ Enfocament Catòlic:** El sistema està instruït per evitar judicis moralitzants, oferint en el seu lloc acollida, sentit espiritual i referències bíbliques d'esperança.
*   **🌐 Bilingüe:** Funciona nativament en Espanyol i Català.

**Com s'utilitza?**
1.  Seleccioneu el vostre idioma a la pantalla de benvinguda.
2.  Escriviu o dicteu la vostra experiència o inquietud actual.
3.  (Opcional) Adjunteu un fitxer (PDF/Word) si voleu que el sistema analitzi un text llarg.
4.  Rebeu una reflexió estructurada en quatre punts: Acollida, Sentit, Orientació i Paraula de Vida.

---

### II. Documentació Tècnica (Enginyeria)
**Arquitectura del Sistema**
El backend està construït en **Rust** utilitzant `Actix-Web` per garantir el màxim rendiment, seguretat de memòria i concurrència. El frontend és HTML5/CSS3 pur amb renderitzat en servidor (SSR) via `Tera`.

**Requisits Previs**
*   **Rust & Cargo:** v1.84 o superior.
*   **Llibreries del Sistema:** `pkg-config`, `libssl-dev`, `libpoppler-glib-dev` (per processament de PDFs).
*   **API Key:** Una clau vàlida d'OpenAI (`OPENAI_API_KEY`).

**Instal·lació i Desplegament**

1.  **Configuració d'Entorn:**
    Creeu un fitxer `.env` a l'arrel:
    ```env
    OPENAI_API_KEY=sk-la-vostra-clau-aqui...
    AI_MODEL=gpt-4o-mini
    PORT=8080
    ```

2.  **Execució Local:**
    ```bash
    cargo run --release
    ```
    Accediu a: `http://localhost:8080`

3.  **Docker (Producció):**
    El projecte inclou un `Dockerfile` *multi-stage* optimitzat (basat en Debian Slim).
    ```bash
    docker build -t acompanyant-espiritual .
    docker run -p 8080:8080 --env-file .env acompanyant-espiritual
    ```

**Enginyeria de Prompts**
El sistema utilitza injecció de prompts dinàmics segons l'idioma seleccionat (`get_system_prompt`), forçant una estructura HTML estricta en la resposta del LLM per garantir el format de sortida.

---

<p align="center">
  Designed by <strong>Angel A. Urbina</strong> &copy; 2026<br>
  <em>Ad Majorem Dei Gloriam</em>
</p>