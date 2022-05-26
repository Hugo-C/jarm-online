<template>
  <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Material+Icons|Material+Icons+Outlined">
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;900&display=swap"
        rel="stylesheet">
  <link rel="stylesheet"
        href="https://cdnjs.cloudflare.com/ajax/libs/github-fork-ribbon-css/0.2.3/gh-fork-ribbon.min.css"/>
  <a id="darkRibbon" class="github-fork-ribbon right-top" href="https://github.com/simonwhitaker/github-fork-ribbon-css"
     data-ribbon="Fork me on GitHub" title="Fork me on GitHub">Fork me on GitHub</a>
  <form class="searchBarDiv" @submit="onSubmit">
    <it-input v-model="inputUrl" prefix-icon="search" label-top="Compute JARM hash" placeholder="Url"
              autofocus ref="inputBar"/>
    <p id="disclaimerLine">
      <it-tag id="disclaimerTag" type="primary" filled>Disclaimer:</it-tag>
      the URL and its hash are saved and displayed publicly
    </p>
  </form>
  <div v-if="computingJarmHash || jarmHashResult.hash">
    <it-divider/>
    <it-progressbar infinite v-if="computingJarmHash"/>
    <div v-if="jarmHashResult.hash">
      <div class="hashDisplay">
        Jarm hash is: <b>{{ jarmHashResult.hash }}</b>
        <it-popover placement="right">
          <it-button @click="copy" id="copyButton">
            <it-icon name="content_copy" color="#000"/>
          </it-button>
          <template #content>Copied!</template>
        </it-popover>
      </div>
      <it-divider/>
      <div>
        Alexa top 1 Million overlap:
        <it-tooltip content="Star the github repo to see new releases in your feed" placement="bottom">
          <a href="https://github.com/" target="_blank" rel="noopener noreferrer">
            <it-tag type="black" filled>not yet implemented</it-tag>
          </a>
        </it-tooltip>
        <it-divider vertical/>
        Known malicious malware family:
        <it-tooltip content="Star the github repo to see new releases in your feed" placement="bottom">
          <a href="https://github.com/" target="_blank" rel="noopener noreferrer">
            <it-tag type="black" filled>not yet implemented</it-tag>
          </a>
        </it-tooltip>
      </div>
    </div>
  </div>
  <div id="footer">
    <it-divider/>
    <h4> Latest urls requested
      <it-tooltip content="Star the github repo to see new releases in your feed" placement="bottom">
        <a href="https://github.com/" target="_blank" rel="noopener noreferrer">
          <it-tag type="black" filled>not yet implemented</it-tag>
        </a>
      </it-tooltip>
    </h4>
    <it-collapse>
      <it-collapse-item v-for='index in 5' :key='index' :title="'URL ' + index">
         JARM and it's maliciousness about URL {{ index }}
      </it-collapse-item>
    </it-collapse>
  </div>
</template>

<script>
import axios from 'axios';
import useClipboard from 'vue-clipboard3'

export default {
  data() {
    return {
      jarmHashResult: {
        hash: null,
      },
      computingJarmHash: false
    }
  },
  methods: {
    onSubmit(evt) {
      evt.preventDefault();
      this.computingJarmHash = true;
      this.jarmHashResult.hash = null;
      this.lookUpUrl(this.inputUrl)
    },
    lookUpUrl(url) {
      const path = 'http://localhost:8000/jarm';
      const payload = {
        params: {
          host: url,
        }
      };
      axios.get(path, payload)
          .then((res) => {
            this.jarmHashResult.hash = res.data.jarm_hash;
            this.computingJarmHash = false;
          })
          .catch((error) => {
            this.$Notification.danger({
              title: 'Failed to query the API',
              text: error,
            })
            this.computingJarmHash = false;
          });
    },
    async copy() {
      try {
        await useClipboard().toClipboard(this.jarmHashResult.hash)
      } catch (e) {
        console.error(e)
      }
    }
  }
}

</script>

<style>
.searchBarDiv {
  padding-top: 20px;
  padding-bottom: 45px;
  width: 66%;
  margin: 0 auto;
}

.hashDisplay {
  font-size: 125%;
}

#copyButton {
  margin: 3px
}

#disclaimerLine {
  margin-top: 3px;
}

#disclaimerTag {
  font-size: 100%;
}

#darkRibbon:before {
  background-color: #333;
}

#footer {
  bottom: 50px;
  position: fixed;
  width: 75%;
  margin-left: 10%;
  margin-right: 10%;
}
</style>
