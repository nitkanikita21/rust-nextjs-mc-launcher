import styles from "@/styles/ui/DialogContainer.module.scss";
import classNames from "classnames";
import React from "react";


export type DialogContainerProps = {
  active?: boolean
  title: string
  children?: React.ReactNode
}

export default function DialogContainer({ children, title, active }: DialogContainerProps) {
  return <>
    {active ? <div className={classNames(styles.bg, styles.centered, styles.root)}>
      <div className={classNames(styles.box)}>
        <h2 className={styles.box_title}>
          {title}
        </h2>
        <div className={styles.box_content}>
          {children}
        </div>
      </div>
    </div> : <></>}
  </>;
}