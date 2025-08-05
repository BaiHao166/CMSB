import Login from "../components/Login.vue";
import App from "../App.vue";
import WorkSpace from "../components/WorkSpace.vue";
const routes = [
    {
        path: "/",
        component: App,
        children: [
            {
                path: "",
                redirect: "login"
            },
            {
                path: "login",
                name: "login",
                component: Login
            },
            {
                path: "workspace",
                name: "workspace",
                component: WorkSpace
            }
        ]
    }
]

export default routes;