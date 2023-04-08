import classNames from "classnames";

import styles from "@/styles/home/ControllPanel.module.scss";

import Button, { ButtonStyles } from "../ui/widgets/Button";


export default function ControllPanel() {


  return <>
    <div className={classNames(styles.container, "offsets")}>
      <Button text="Грати" style={ButtonStyles.GREEN} />
    </div>
  </>;
}