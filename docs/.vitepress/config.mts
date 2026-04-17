import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Funky",
  description: "Turn command history into reusable shell functions.",
  base: "/funky/",
  cleanUrls: true,
  srcExclude: ["workflows/**"],

  head: [
    ["link", { rel: "icon", type: "image/svg+xml", href: "/funky/funky-icon.svg" }],
    ["meta", { name: "theme-color", content: "#7c3aed" }],
    [
      "meta",
      { property: "og:description", content: "Turn command history into reusable shell functions." },
    ],
  ],

  themeConfig: {
    nav: [
      { text: "Guide", link: "/guide/installation" },
      { text: "Reference", link: "/reference/commands" },
    ],

    sidebar: [
      {
        text: "Guide",
        items: [
          { text: "Installation", link: "/guide/installation" },
          { text: "Getting Started", link: "/guide/getting-started" },
          { text: "Creating Functions", link: "/guide/creating-functions" },
          { text: "Managing Functions", link: "/guide/managing-functions" },
          { text: "Sources", link: "/guide/sources" },
          { text: "Supported Shells", link: "/guide/shells" },
        ],
      },
      {
        text: "Commands",
        items: [
          { text: "Overview", link: "/reference/commands" },
          { text: "init", link: "/reference/init" },
          { text: "new", link: "/reference/new" },
          { text: "list", link: "/reference/list" },
          { text: "edit", link: "/reference/edit" },
          { text: "usage", link: "/reference/usage" },
          { text: "Configuration", link: "/reference/configuration" },
        ],
      },
      {
        text: "Development",
        items: [
          { text: "Contributing", link: "/development/contributing" },
          { text: "Architecture", link: "/development/architecture" },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/KyleChamberlin/funky" },
    ],

    editLink: {
      pattern: "https://github.com/KyleChamberlin/funky/edit/main/docs/:path",
    },

    footer: {
      message: "Released under the GPL-3.0 License.",
      copyright: "Copyright © Kyle Chamberlin",
    },

    search: {
      provider: "local",
    },
  },
});
