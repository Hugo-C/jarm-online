<template>
  <metainfo>
    <template v-slot:title="{ content }">{{`${content}` }}</template>
  </metainfo>
  <MainPage/>
</template>

<script>
import { useMeta } from 'vue-meta'
import MainPage from './components/MainPage.vue'

export default {
  name: 'App',
  components: {
    MainPage
  },
  setup () {
    useMeta({
      title: 'Jarm online',
      htmlAttrs: {lang: 'en'},
      meta: [
        {charset: 'utf-8'},
        {name: 'description', content: 'Compute jarm hash online with this cybersecurity tool. List of malicious hashes included. Disclaimer: results are saved in database'},
      ]
    })
  },
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