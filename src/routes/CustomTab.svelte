<script>
  import { page } from '$app/stores';
  export let href = '';
</script>

<a class="tab tab-lifted" class:tab-active={$page.url.pathname === href} href={href}><b><slot /></b></a>