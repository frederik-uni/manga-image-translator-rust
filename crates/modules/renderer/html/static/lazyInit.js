if (typeof inited === "undefined") {
  if (typeof maxWidth !== "undefined") {
    document.querySelector("body").style.maxWidth = maxWidth;
  }
  const viewportMeta = document.createElement("meta");
  viewportMeta.setAttribute("name", "viewport");
  viewportMeta.setAttribute("content", "width=device-width, initial-scale=1.0");
  document.head.appendChild(viewportMeta);
  const jqueryScript = document.createElement("script");
  jqueryScript.src = "https://code.jquery.com/jquery-3.7.1.min.js";

  jqueryScript.onload = () => {
    const fitTextScript = document.createElement("script");
    fitTextScript.src =
      "https://cdnjs.cloudflare.com/ajax/libs/FitText.js/1.2.0/jquery.fittext.min.js";

    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.href = "/style.css";
    document.head.appendChild(link);

    fitTextScript.onload = () => {
      const script = document.createElement("script");
      script.src = "/script.js";
      document.head.appendChild(script);
    };

    document.head.appendChild(fitTextScript);
  };

  document.head.appendChild(jqueryScript);
}
