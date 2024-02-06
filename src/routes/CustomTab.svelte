<script>
  import { page } from '$app/stores';
  export let href = '';

  $: tabIsActive = href == '/' ? $page.url.pathname == '/' : $page.url.pathname.includes(href) ;
</script>

<a class="tab tab-lifted " class:tab-active={tabIsActive} href={href}><slot /></a>