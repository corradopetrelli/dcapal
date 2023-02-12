import React from "react";
import { CcyRadio } from "./ccyRadio";

export const CcyGroup = ({ ccys, selected, setSelected, ...props }) => (
  <div className="flex flex-wrap gap-2" data-testid="ccy-group">
    {ccys.map((c) => (
      <CcyRadio key={c} ccy={c} selected={selected} setSelected={setSelected} />
    ))}
  </div>
);
