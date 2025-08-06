<script setup>
import { defineEmits } from "vue";
import { menuFlag } from "../js/constants"
const emit = defineEmits(["clickOnceMenu"]);
function clickMenu(menuName) {
    emit("clickOnceMenu", menuName)
}
</script>

<template>
    <div class="side-menu">
        <ul>
            <li @click="clickMenu(menuFlag.list)">
                列表
            </li>
            <li @click="clickMenu(menuFlag.uploadList)">
                上传记录
            </li>
            <li @click="clickMenu(menuFlag.downloadList)">
                下载记录
            </li>
            <li @click="clickMenu(menuFlag.setting)">
                设置
            </li>
        </ul>
    </div>
</template>

<style scoped>
.side-menu {
    width: 95%;
    height: 90%;
    margin-top: 1%;
    padding: 5px;
}

ul {
    list-style: none;
    padding: 0;
    margin: 0;
}

ul>li {
    padding: 5px 20px;
    margin-top: 5px;
    border-radius: 15px;
}

ul>li:hover {
    background-color: rgba(154, 196, 236, 0.3);
    cursor: pointer;
}
</style>