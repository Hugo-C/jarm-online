<template>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;900&display=swap"
        rel="stylesheet">
  <link href="https://fonts.googleapis.com/css?family=Material+Icons" rel="stylesheet">
  <link rel="stylesheet"
        href="https://cdnjs.cloudflare.com/ajax/libs/github-fork-ribbon-css/0.2.3/gh-fork-ribbon.min.css"/>
  <a id="darkRibbon" class="github-fork-ribbon right-top" href="https://github.com/Hugo-C/jarm-online"
     data-ribbon="Fork me on GitHub" title="Fork me on GitHub">Fork me on GitHub</a>
  <form class="searchBarDiv" @submit="onSubmit">
    <v-text-field
        variant="solo-inverted"
        label="Compute JARM hash"
        placeholder="8.8.8.8 | host.com/path"
        prepend-inner-icon="search"
        autofocus
        loading
        @click:prepend-inner="onSubmit"
        v-model="inputUrl"
    >
      <template v-slot:loader>
        <v-progress-linear
            :active="computingJarmHash"
            color="secondary"
            absolute
            rounded
            height="5"
            indeterminate
        ></v-progress-linear>
      </template>
    </v-text-field>
    <p id="disclaimerLine">
      <v-chip label variant="elevated" color="primary">Disclaimer:</v-chip>
      the URL and its hash are saved and displayed publicly
    </p>
  </form>
  <v-expand-transition>
    <div v-if="jarmHashResult.hash">
      <div class="hashDisplay">
        <v-card variant="outlined" class="mx-auto pa-5" width="70%">
          JARM hash is: <b size="large">{{ jarmHashResult.hash }}</b>
          <v-btn @click="copy" variant="text" prepend-icon="content_copy" class="ml-2" size="small"
                 stacked>
            Copy Me
            <v-tooltip :open-on-hover="false" :open-on-click="true" :no-click-animation="true" text="Copied!"
                       activator="parent"/>
          </v-btn>
        </v-card>
      </div>
      <div>
        Alexa top 1 Million overlap:
        <a href="https://github.com/Hugo-C/jarm-online" target="_blank" rel="noopener noreferrer">
          <v-chip label size="small" class="ma-1" variant="elevated" color="info">Not yet implemented
            <v-tooltip
                text="Star the github repo to see new releases in your feed"
                location="bottom" activator="parent"/>
          </v-chip>
        </a>
        <v-divider
            vertical
            color="info"
            :thickness="2"
            class="ma-1 border-opacity-100"
        ></v-divider>
        Known malicious malware family:
        <a href="https://github.com/Hugo-C/jarm-online" target="_blank" rel="noopener noreferrer">
          <v-chip label size="small" class="ma-1" variant="elevated" color="info">Not yet implemented
            <v-tooltip
                text="Star the github repo to see new releases in your feed"
                location="bottom" activator="parent"/>
          </v-chip>
        </a>
      </div>
    </div>
  </v-expand-transition>

  <div id="footer">
    <v-divider :thickness="2" class="border-opacity-100" color="info" inset/>
    <h4> Latest urls requested
      <a href="https://github.com/Hugo-C/jarm-online" target="_blank" rel="noopener noreferrer">
        <v-chip label size="small" class="ma-1" variant="elevated" color="info">Not yet implemented
          <v-tooltip
              text="Star the github repo to see new releases in your feed"
              location="bottom" activator="parent"/>
        </v-chip>
      </a>
    </h4>
    <!--    TODO use https://vuetifyjs.com/en/components/expansion-panels/-->
  </div>
</template>

<script>
import axios from 'axios';
import useClipboard from 'vue-clipboard3'

export default {
  data() {
    return {
      inputUrl: null,
      jarmHashResult: {
        hash: null,
      },
      computingJarmHash: false
    }
  },
  methods: {
    async onSubmit(evt) {
      evt.preventDefault();
      this.jarmHashResult.hash = null;  // Force reset
      this.jarmHashResult.hash = await this.lookUpUrl(this.inputUrl)
    },
    async lookUpUrl(url) {
      let jarm_hash = null;
      this.computingJarmHash = true;
      const path = '/api/v1/jarm';
      const payload = {
        params: {
          host: url,
        }
      };
      try {
        const res = await axios.get(path, payload);
        if (res.data.error) {
          // this.$Notification.danger({
          //   title: 'API returned an error',
          //   text: res.data.error.error_type,
          // })
        } else {
          jarm_hash = res.data.jarm_hash;
        }
      } catch (error) {
        // this.$Notification.danger({
        //   title: 'Failed to query the API',
        //   text: error,
        // })
      }
      this.computingJarmHash = false;
      return jarm_hash;
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
