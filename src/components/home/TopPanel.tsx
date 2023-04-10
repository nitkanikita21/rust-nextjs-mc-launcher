import classNames from "classnames";
import styles from "@/styles/home/TopPanel.module.scss";
import Login from "./topPanel/Login";
import Settings from "./topPanel/Settings";


export default function TopPanel() {

  return <>
    <div className={classNames("offsets", styles.container)}>
      <div className={classNames(styles.launcher_name)}>
        Nitka&lsquo;s Launcher
      </div>

      <div className={styles.button_container}>
        <Settings/>
        <Login/>
      </div>
    </div>
  </>;
}