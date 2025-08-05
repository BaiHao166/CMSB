import { reactive  } from "vue";

const store = reactive({
    userinfo: null,
    isLoggedIn: false
})

export function setUserInfo(userInfo) {
    store.userinfo = Object.freeze(userInfo);
    store.isLoggedIn = true;
}

export function getUserInfo() {
    return store.userinfo;
}

export default store;
