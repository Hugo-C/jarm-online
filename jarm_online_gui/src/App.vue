<template>
  <Head>
    <title>Jarm online</title>
    <meta name="description" content="Compute jarm hash online with this cybersecurity tool. List of malicious hashes included. Disclaimer: results are saved in database">
  </Head>
  <MainPage />
</template>

<script>
import MainPage from './components/MainPage.vue'

export default {
  name: 'App',
  components: {
    MainPage
  },
  setup() {},
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>