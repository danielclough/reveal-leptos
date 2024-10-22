Fabric

```yaml
fabric: 
    color_number: 3
    colors: ["matcha_green","lime_green","blue_green"]
    fabrics: 
        -
            matcha_green: 
                yardage_in: 49.625
                wof_in: 44
                sub_cut: 
                    -
                        name: "wide_diamond"
                        strips: 13
                        strip_width_in: 3.125
                    -
                        name: "skinny_diamond"
                        strips: 4
                        strip_width_in: 2.25
        -
            lime_green: 
                yardage_in: 13.875
                wof_in: 44
                sub_cut: 
                    -
                        name: "wide_diamond"
                        strips: 3
                        strip_width_in: 3.125
                    -
                        name: "skinny_diamond"
                        strips: 2
                        strip_width_in: 2.25
        -
            blue_green: 
                yardage_in: 17.5
                wof_in: 44
                sub_cut: 
                    -
                        name: "wide_diamond"
                        strips: 2
                        strip_width_in: 3.125
                    -
                        name: "skinny_diamond"
                        strips: 5
                        strip_width_in: 2.25
```


Shapes

```yaml

wide_diamonds: 
    qty: 
    colors: 

skinny_diamonds: 
    qty: 
    colors: 

units: 

```


# Quilt Shape Math

```yaml
diamond_type:
  - name: wide_diamond
    remaining_tri_in: .75
    diamond_side: 2.5
    total_length_of_fabric: 43.5
    seam_allowance: 3/8
    skew_allowance: 3/8
    skinny_angle: 72
    skew_allowance: (seam_allowance / tan(skinny_angle / 2))
  - name: skinny_diamond
    remaining_tri_in: 2.125
    diamond_side: 2.5
    total_length_of_fabric: 43.5
    seam_allowance: 5/16
    skinny_angle: 36
    skew_allowance: (seam_allowance / tan(skinny_angle / 2))
```
(total_length_of_fabric - remaining_tri_in) / (diamond_side + (seam_allowance \* 2))