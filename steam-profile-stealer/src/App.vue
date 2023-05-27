<script setup lang="ts">
import { useCookieStore } from "@/stores/cookie";
import CookieInput from "@/components/CookieInput.vue";
import type { Profile } from "@/stores/profile";
import { useProfileStore } from "@/stores/profile";
import Console from "@/components/Console.vue";
import ProfileComponent from "@/components/profiles/ProfileComponent.vue";
import { ref } from "vue";

const cookieStore = useCookieStore();
const messages = ref([]);
const ws = new WebSocket('ws://localhost:8000/ws');
const profileStore = useProfileStore();

export type SteamMessageIn =
    | { tag: "status_update", fields: { message: string } }
    | { tag: "self_profile", fields: { profile: Profile } }
    | { tag: "profile_fetch", fields: { profile: Profile } }
    | { tag: "error", fields: { message: string } }
    | { tag: "name_change", fields: { name: string } }
    | { tag: "picture_change", fields: { url: string } };

export type SteamMessageOut =
    | { tag: "cookie", fields: { cookie: string } }
    | { tag: "refresh_profile" }
    | { tag: "steal_profile", fields: { name: string, image_url: string } }
    | { tag: "fetch_profile", fields: { url: string } }

function send(object: SteamMessageOut) {
  if (ws.readyState === ws.OPEN) {
    ws.send(JSON.stringify(object));
  }
}

ws.addEventListener('open', () => {
  console.log('websocket opened!');
  if (cookieStore.cookie) {
    send({tag: "cookie", fields: {cookie: cookieStore.cookie}});
  }
});

ws.addEventListener('close', c => {
  console.log('websocket closed', c);
});

ws.addEventListener('error', e => {
  console.error(e);
})


ws.addEventListener('message', ({data}) => {
  const j = JSON.parse(data) as SteamMessageIn;
  console.log(j);

  if (j.tag === 'error' || j.tag === 'self_profile' || j.tag === 'profile_fetch' || j.tag === 'picture_change') {
    loading.value = false;
  }

  switch (j.tag) {
    case 'status_update':
    case 'error':
      const message = j.fields.message;
      messages.value.push({message, key: Math.random().toString()});
      if(messages.value.length > 10) {
        messages.value.shift();
      }

      break;
    case 'self_profile':
      profileStore.selfProfile = j.fields.profile;
      break;
    case 'profile_fetch':
      profileStore.targetProfile = j.fields.profile;
      break;
    case 'name_change':
      if (profileStore.selfProfile) {
        profileStore.selfProfile.name = j.fields.name;
      }
      break;
    case 'picture_change':
      if (profileStore.selfProfile) {
        profileStore.selfProfile.image_url = j.fields.url;
      }

      profileStore.targetProfile = null;
      break;
  }
});
let loading = ref(false);
let targetProfile = ref('');

function refreshProfile() {
  loading.value = true;

  send({tag: 'refresh_profile'})
}

function fetchProfile() {
  const url = targetProfile.value;
  if (!url) return;

  loading.value = true;
  send({tag: 'fetch_profile', fields: {url}})
}

function stealProfile() {
  const target = profileStore.targetProfile;
  if (!target) return;

  loading.value = true;
  send({tag: 'steal_profile', fields: {name: target.name, image_url: target.image_url}})
}
</script>


<template>
  <v-app>
    <v-main>
      <CookieInput v-if="!cookieStore.cookie || !profileStore.selfProfile"
                   v-on:saveCookie="send({tag: 'cookie', fields: {cookie: cookieStore.cookie!}})"/>
      <v-container v-else>
        <ProfileComponent v-bind="profileStore.selfProfile"></ProfileComponent>
        <ProfileComponent v-if="profileStore.targetProfile" v-bind="profileStore.targetProfile"></ProfileComponent>
        <v-progress-circular indeterminate v-if="loading"></v-progress-circular>

        <v-btn :disabled="loading" @click="refreshProfile">Refresh Profile</v-btn>

        <v-text-field
            label="Target Profile"
            v-model="targetProfile"
            :disabled="loading"
        ></v-text-field>

        <v-btn :disabled="loading" @click="fetchProfile">Fetch Target Profile</v-btn>

        <v-btn :disabled="loading" v-if="profileStore.targetProfile" @click="stealProfile">Steal Profile</v-btn>

        <Console :messages="messages"></Console>
      </v-container>
    </v-main>
  </v-app>
</template>
