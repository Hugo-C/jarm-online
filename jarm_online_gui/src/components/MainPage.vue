<template>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;900&display=swap"
        rel="stylesheet">
  <link href="https://fonts.googleapis.com/css?family=Material+Icons" rel="stylesheet">
  <link rel="stylesheet"
        href="https://cdnjs.cloudflare.com/ajax/libs/github-fork-ribbon-css/0.2.3/gh-fork-ribbon.min.css"/>
  <a id="darkRibbon" class="github-fork-ribbon right-top" href="https://github.com/Hugo-C/jarm-online"
     data-ribbon="Fork me on GitHub" title="Fork me on GitHub">Fork me on GitHub</a>
  <v-layout>
    <v-main>
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
                rounded
                height="5"
                indeterminate
            ></v-progress-linear>
          </template>
        </v-text-field>
        <p id="disclaimerLine">
          <v-chip label variant="elevated" color="primary-darken-1">Disclaimer:</v-chip>
          the URL and its hash are saved and displayed publicly
        </p>
      </form>
      <v-expand-transition>
        <div v-if="jarmHashResult.hash">
          <div class="hashDisplay">
            <v-card variant="outlined" class="mx-auto pa-5" width="70%">
              JARM hash is: <b size="large">{{ jarmHashResult.hash }}</b>
              <v-btn @click="copy" variant="text" prepend-icon="content_copy" class="ml-2" size="small" stacked>
                Copy Me
                <v-tooltip :open-on-hover="false" :open-on-click="true" :no-click-animation="true" text="Copied!"
                           activator="parent"/>
              </v-btn>
            </v-card>
          </div>
          <v-container fill-height class="w-75">
            <v-row align="center" justify="center">
              <div>
                Alexa top 1 Million overlap:
                <v-progress-circular
                    v-if="computingAlexaRank"
                    indeterminate
                    color="secondary"
                ></v-progress-circular>
                <span v-else-if="this.jarmHashResult.alexa.topRank != null">
          <v-chip label variant="elevated" color="primary">{{ this.jarmHashResult.alexa.topRank }}th Rank</v-chip>
          <b class="pa-2" size="large"> {{ this.jarmHashResult.alexa.topDomain }}</b>
          <a
              v-if="this.jarmHashResult.alexa.raw_result.overlapping_domains.length > 1"
              :href="'/api/v1/alexa-overlap?jarm_hash=' + jarmHashResult.hash">
            See {{
              this.jarmHashResult.alexa.raw_result.overlapping_domains.length - 1
            }} other matching domains</a>
        </span>
                <span v-else>
          <v-chip label variant="elevated" color="primary">No match found</v-chip>
        </span>
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
              <v-expansion-panels class="pa-2">
                <v-expansion-panel>
                  <v-expansion-panel-title id="shodanPanel">
                    Shodan
                    <!-- TODO add a result count like https://api.shodan.io/shodan/host/count?query=ssl.jarm:29d29d00029d29d00042d42d00000000f78d2dc0ce6e5bbc5b8149a4872356 -->
                    <!--                <div v-if="computingShodanResultCount" class="w-50">-->
                    <!--                  <v-progress-linear indeterminate color="primary" :absolute="true"></v-progress-linear>-->
                    <!--                </div>-->
                    <!--                <span v-else class="shodanResultCount ">-->
                    <!--                    <v-chip label variant="elevated" class="mr-5" color="primary">70</v-chip>-->
                    <!--                </span>-->
                  </v-expansion-panel-title>
                  <v-expansion-panel-text>
                    <a :href="shodanSearchLink" target="_blank" rel="noopener noreferrer">
                      <v-img :src="shodanImageLink" lazy-src="/shodan_placeholder.png" height="480">
                        <template v-slot:placeholder>
                          <div class="d-flex align-center justify-center fill-height">
                            <v-progress-circular
                                color="grey-lighten-4"
                                indeterminate
                            ></v-progress-circular>
                          </div>
                        </template>
                      </v-img>
                    </a>
                  </v-expansion-panel-text>
                </v-expansion-panel>
              </v-expansion-panels>
            </v-row>
          </v-container>
        </div>
      </v-expand-transition>

      <!-- Latest URLs part -->
      <v-container fill-height class="w-75">
        <v-divider :thickness="2" class="border-opacity-100" color="info" inset/>
        <h4> Latest urls requested</h4>
        <v-progress-circular
            v-if="lastScans === null"
            indeterminate
            color="secondary"
        ></v-progress-circular>
        <v-expansion-panels v-else multiple fill-width>
          <v-slide-y-reverse-transition
              class="py-0"
              group
              tag="v-expansion-panel"
          >
            <v-expansion-panel
                v-for="scan in lastScans.slice().reverse()"
                :key="scan.host"
                class="ma-1"
                eager
            >
              <v-expansion-panel-title>
                <span>{{ scan.host }}:{{ scan.port }}</span>
              </v-expansion-panel-title>
              <v-expansion-panel-text>
                <span>{{ scan.jarm_hash }}</span>
              </v-expansion-panel-text>
            </v-expansion-panel>
          </v-slide-y-reverse-transition>
        </v-expansion-panels>
      </v-container>
    </v-main>
    <v-footer elevation="2" app>
      <v-row justify="center" no-gutters>
        <h5 id="footernote">
          hosted with 💙 on
          <v-img id="gcpimg" src="gcp.png" height="20" width="20" inline></v-img>
          <span class="ma-5"> |</span>
          <a href="https://jarm.statuspage.io/" target="_blank" rel="noopener noreferrer">status page</a>
        </h5>
      </v-row>
    </v-footer>
  </v-layout>
  <!--  Snackbar for notifications-->
  <v-snackbar
      v-model="this.notification.isDisplayed"
      :timeout="10000"
      variant="flat"
      color="error"
      :absolute="true"
      class="ma-5 opacity-100"
      location="top right"
      z-index="15000"
      multi-line
      vertical
  >
    <div class="text-subtitle-1 pb-2">{{ notification.title }}</div>
    <p>{{ notification.body }}</p>
    <template v-slot:actions>
      <v-btn
          variant="text"
          @click="notification.clear()"
      >
        Close
      </v-btn>
    </template>
  </v-snackbar>
</template>

<script>
import axios from 'axios';
import useClipboard from 'vue-clipboard3'

import {notification} from './notification';

export default {
  data() {
    return {
      inputUrl: null,
      jarmHashResult: {
        hash: null,
        alexa: {
          raw_result: null,
          topRank: null,
          topDomain: null,
        },
      },
      computingJarmHash: false,
      computingAlexaRank: false,
      notification: notification,
      shodanImageLink: null,
      shodanSearchLink: null,
      lastScans: null,
    }
  },
  async mounted() {
    await this.fetchLastScans()
  },
  methods: {
    resetJarmHash() {
      this.jarmHashResult.hash = null;
      this.jarmHashResult.alexa.raw_result = null;
      this.jarmHashResult.alexa.topRank = null;
      this.jarmHashResult.alexa.topDomain = null;
      this.shodanImageLink = null;
      this.shodanSearchLink = null;
    },
    setJarmHash(hash) {
      this.jarmHashResult.hash = hash;
      if (!hash) {
        return;
      }
      this.shodanImageLink = `https://www.shodan.io/search/facet.png?query=ssl.jarm%3A${hash}&facet=product`;
      this.shodanSearchLink = `https://www.shodan.io/search?query=ssl.jarm:${hash}`;
    },
    async onSubmit(evt) {
      evt.preventDefault();
      this.resetJarmHash();

      let hash = await this.lookUpUrl(this.inputUrl);
      this.setJarmHash(hash);
      if (!hash) {
        return;  // Skip alexa as no hash was returned
      }
      this.jarmHashResult.alexa.raw_result = await this.alexaOverlap(this.jarmHashResult.hash);
      // Parse alexa result
      this.jarmHashResult.alexa.topRank = this.jarmHashResult.alexa.raw_result.overlapping_domains[0].rank
      this.jarmHashResult.alexa.topDomain = this.jarmHashResult.alexa.raw_result.overlapping_domains[0].domain
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
          this.notification.display(
              'API returned an error',
              res.data.error.error_type,
          );
        } else {
          jarm_hash = res.data.jarm_hash;
        }
      } catch (error) {
        this.notification.display(
            'Failed to query the API',
            error,
        );
      }
      this.computingJarmHash = false;
      return jarm_hash;
    },
    async alexaOverlap(hash) {
      this.computingAlexaRank = true;
      const path = '/api/v1/alexa-overlap';
      const payload = {
        params: {
          jarm_hash: hash,
        }
      };
      let result;
      try {
        const res = await axios.get(path, payload);
        result = res.data
      } catch (error) {
        this.notification.display(
            'Failed to query the API',
            error,
        );
      }
      this.computingAlexaRank = false;
      return result;
    },
    async fetchLastScans() {
      const path = '/api/v1/last-scans';
      try {
        const res = await axios.get(path);
        this.lastScans = res.data.last_scans;
      } catch (error) {
        this.notification.display(
            'Failed to query the API for last scans',
            error,
        );
      }
      setTimeout(this.fetchLastScans, 5000);  // Refresh in 5s
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
.v-expansion-panel {
    min-width: 700px;
}
.searchBarDiv {
  padding-top: 20px;
  padding-bottom: 45px;
  width: 66%;
  margin: 0 auto;
}

.hashDisplay {
  font-size: 125%;
}

#shodanPanel {
  display: flex;
}

.shodanResultCount {
  flex: 1;
  text-align: right;
  align-content: flex-end;
  white-space: nowrap;
}

#disclaimerLine {
  margin-top: 3px;
}

#darkRibbon:before {
  background-color: #333;
}

#footernote {
  color: dimgray;
}

#gcpimg {
  vertical-align: -20%;
}
</style>
