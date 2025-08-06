<script setup>
import { defineProps, computed } from "vue";
import { menuFlag } from "../js/constants"
import PersionList from "./PersionList.vue"
const props = defineProps({
    menu: {
        type: String,
        required: true,
        default: ""
    }
})



let componentName = computed(() => {
    switch (props.menu) {
        case menuFlag.list:
            return PersionList;
        case menuFlag.uploadList:
            return "About";
        case menuFlag.downloadList:
            return "Contact";
        case menuFlag.setting:
            return "Contact";
    }
});

</script>

<template>
    <div class="right-area">
        <component :is="componentName"></component>
    </div>
</template>

<style scoped>
.right-area {
    width: 77.5%;
    height: 99.5%;
    background-color: white;
    border-radius: 10px;
    margin-left: 0.8%;
    padding: 4px;
}
</style>