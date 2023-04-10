import classNames from "classnames";

import styles from "@/styles/home/ControllPanel.module.scss";

import ProgressBar from "../ui/widgets/ProgressBar";


export default function ControllPanel() {


  return <>
    <div className={classNames(styles.container, "offsets")}>
      {/* <Button text="Грати" style={ButtonStyles.GREEN} /> */}
      <ProgressBar id="test"/>
    </div>
  </>;
}