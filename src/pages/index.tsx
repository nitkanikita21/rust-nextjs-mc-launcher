import TopPanel from "@/components/home/TopPanel";
import classNames from "classnames";
import ControllPanel from "@/components/home/ControllPanel";

import styles from "@/styles/pages/Home.module.scss";


export default function Home() {

  return (
    <>
      <div className={classNames("full-height", styles.vertical_container)}>
        <TopPanel />
        {/* <PacksContainer></PacksContainer> */}
        <ControllPanel></ControllPanel>
      </div>

    </>
  );
}
