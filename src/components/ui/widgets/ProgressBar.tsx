import styles from "@/styles/ui/widgets/ProgressBar.module.scss";
import { listen } from "@tauri-apps/api/event";
import classNames from "classnames";
import { useEffect, useState } from "react";

export type ProgressBarProps = {
  id: string,
}

export type ProgressBarDisplay = {
  title: string,
  displayValue: string
}

export type ProgressBarState = {
  value: number,

  display: ProgressBarDisplay
}

type ProgressBarChangeState = {
  barId: string,
  state: ProgressBarState
}

export default function ProgressBar(
  {
    id
  }: ProgressBarProps
) {
  const [barState, setBarState] = useState<ProgressBarState>({
    value: 0,
    display: {
      title: "",
      displayValue: ""
    }
  });

  useEffect(() => {
    listen<ProgressBarChangeState>("progress-bar-state-change", (event) => {
      if (event.payload.barId != id) return;
      setBarState(event.payload.state);
    });
  });

  return <>
    <div className={classNames(styles.box, styles.test)}>

      <div className={styles.bar} style={{ "width": `${barState.value}%` }}>
        {barState.value >= 20 ? <h4 className={styles.title}>{barState.display?.title}:</h4> : <></>}
        {barState.value >= 5 ? <p>{barState.display?.displayValue}</p> : <p>&nbsp;</p>}
      </div>

    </div>
  </>;
}