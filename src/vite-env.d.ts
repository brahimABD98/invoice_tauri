// vite-env.d.ts
/// <reference types="vite-plugin-pages/client" />


/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;


}
